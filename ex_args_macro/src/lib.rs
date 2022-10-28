use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs};


fn parse_attrs( attributes : Vec< syn::NestedMeta > ) -> Vec<( syn::Path, proc_macro2::TokenStream )>
{
  attributes.iter().filter_map( | attr |
  {
    if let syn::NestedMeta::Meta( name ) = attr
    {
      if let syn::Meta::List( args_type ) = name.clone()
      {
        let nested_args = args_type.nested;
        return Some(( name.path().to_owned(), quote!( #nested_args ) ));
      }
    }
    None
  }).collect::< Vec< _ > >()
}

fn reconstruct_function( function : &syn::ItemFn, attributes : &Vec<( syn::Path, proc_macro2::TokenStream )> ) -> proc_macro2::TokenStream
{
  let fblock = &function.block;
  let fsig = &function.sig;
  let ( fident, fgenerics, freturn, finputs ) =
  ( &fsig.ident, &fsig.generics, &fsig.output, &fsig.inputs );

  let attrs = attributes.iter()
  .map( | attr |
  {
    let arg_type = &attr.0;
    let arg_generic = &attr.1;
    let name = syn::parse_str::< proc_macro2::TokenStream >
    (
      &quote!( #arg_type ).to_string().to_lowercase()
    ).unwrap();
    quote!( #name : &mut #arg_type< #arg_generic > )
  }).collect::< Vec< _ > >();

  quote!
  (
    fn #fident #fgenerics( #finputs, #(mut #attrs),* ) #freturn
    #fblock
  )
}

fn call_macro
(
  fn_name : &syn::Ident,
  fn_args : &Vec< proc_macro2::TokenStream >,
  extended_args : &Vec<( syn::Path, proc_macro2::TokenStream )>
) -> proc_macro2::TokenStream
{
  let mut impls = Vec::with_capacity( extended_args.len() );
  if let Some( arg ) = extended_args.get( 0 )
  {
    let arg = arg.0.to_owned();
    let name = quote!( #arg ).to_string();
    if &name == "KWARGS"
    {
      impls.push( quote!
      (
        // func!( a, b, hashmap )
        //? "=>" - means that we reuse all after it
        ( #($#fn_args : expr),* => $(;)? $kwargs : expr ) =>
        {
          #fn_name( #($#fn_args,)* $kwargs )
        };
        // func!( a, b, kwargs )
        ( #($#fn_args : expr,)* $($kwargs : tt = $kwvals : expr),+ ) =>
        {
          #fn_name( #($#fn_args,)* &mut HashMap::from([ $((stringify!( $kwargs ).to_owned(), $kwvals)),* ]) )
        };
        // func!( a, b )
        ( #($#fn_args : expr),* ) =>
        {
          #fn_name( #($#fn_args,)* &mut HashMap::default() )
        };
      ))
    }
    else if &name == "ARGS" && extended_args.get( 1 ).is_some()
    {
      impls.push( quote!
      (
        // func!( a, b, hashmap )
        //? "=>" - means that we reuse all after it
        ( #($#fn_args : expr),* => ; $kwargs : expr ) =>
        {
          #fn_name( #($#fn_args,)* &mut vec![], $kwargs )
        };
        // func!( a, b, args, hashmap )
        //? "=>" - means that we reuse all after it
        ( #($#fn_args : expr),* => $args : expr ) =>
        {
          #fn_name( #($#fn_args,)* $args, &mut HashMap::default() )
        };
        // func!( a, b, args, hashmap )
        //? "=>" - means that we reuse all after it
        ( #($#fn_args : expr),* => $args : expr ; $kwargs : expr ) =>
        {
          #fn_name( #($#fn_args,)* $args, $kwargs )
        };
        // func!( a, b, kwargs )
        ( #($#fn_args : expr,)* $($kwargs : tt = $kwvals : expr),+ ) =>
        {
          #fn_name( #($#fn_args,)* &mut vec![], &mut HashMap::from([ $((stringify!( $kwargs ).to_owned(), $kwvals)),* ]) )
        };
        // func!( a, b, args )
        ( #($#fn_args : expr),* $(, $args : expr)* ) =>
        {
          #fn_name( #($#fn_args,)* &mut vec![ $($args),*], &mut HashMap::default() )
        };
        // func!( a, b, args, kwargs )
        ( #($#fn_args : expr,)* $($args : expr),* ; $($kwargs : tt = $kwvals : expr),* ) =>
        {
          #fn_name( #($#fn_args,)* &mut vec![ $($args),*], &mut HashMap::from([ $((stringify!( $kwargs ).to_owned(), $kwvals)),* ]) )
        };
      ))
    }
    else
    {
      impls.push( quote!
      (
        // func!( a, b, args, hashmap )
        //? "=>" - means that we reuse all after it
        ( #($#fn_args : expr),* => $args : expr ) =>
        {
          #fn_name( #($#fn_args,)* $args )
        };
        // func!( a, b, args )
        ( #($#fn_args : expr),* $(, $args : expr)* ) =>
        {
          #fn_name( #($#fn_args,)* &mut vec![ $($args),* ] )
        };
      ))
    }
  }
  let impls = impls.iter().fold( quote!(), | acc, imp |
  {
    quote!( #acc #imp )
  });
  quote!
  (
    macro_rules! #fn_name
    {
      #impls
    }
  )
}

#[ proc_macro_attribute ]
pub fn extend_args( attributes : TokenStream, function : TokenStream ) -> TokenStream 
{
  let mut attributes = parse_attrs( parse_macro_input!( attributes as AttributeArgs ) );
  let function = parse_macro_input!( function as syn::ItemFn );

  // first must be ARGS, second - KWARGS and nothing more
  attributes.sort_by_key( |( path, _ )| quote!( #path ).to_string() );
  let attributes = attributes.iter().filter( |( path, _ )|
  {
    match quote!( #path ).to_string().as_str()
    {
      "ARGS" | "KWARGS" => true,
      _ => false
    }
  }).cloned().collect();

  let farg_names = function.sig.inputs.pairs()
  .map( | input | *input.value() )
  .filter_map( | arg |
  {
    if let syn::FnArg::Typed( pat_type ) = arg
    {
      let arg_name = &pat_type.pat;
      Some( quote!( #arg_name ) )
    }
    else
    { None }
  })
  .collect::< Vec< _ > >();

  let call_macro = call_macro( &function.sig.ident, &farg_names, &attributes );
  let function = reconstruct_function( &function, &attributes );

  let out = quote!
  (
    #function
    #call_macro
  );
  proc_macro::TokenStream::from( out )
}