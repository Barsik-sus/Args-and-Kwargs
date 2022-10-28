use std::io::Write;

use extend_fn_args::*;

#[ extend_args( KWARGS( String ) ) ]
fn news( theme : String )
{
  kwargs.extend
  ([
    ( format!( "{theme}: Theme title" ), "Some news".to_owned() ),
    ( format!( "{theme}: Theme second news" ), "Some other news".to_owned() )]
  )
}

#[ extend_args( ARGS( String ), KWARGS( String ) ) ]
fn format_all_news( f : Box< dyn Write > )
{
  let mut f = f;

  args.iter().for_each( | theme | news!( theme.to_owned() => kwargs ) );

  kwargs.iter()
  .for_each( |( title, description )|
  {
    writeln!( f, "=={title}==\n{description}" ).unwrap();
  });
}

fn main()
{
  format_all_news!( Box::new( std::io::stdout() ), "Programming".to_owned(), "Gaming".to_owned() )
}