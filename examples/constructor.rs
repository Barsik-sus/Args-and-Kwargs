use extend_fn_args::*;

#[ derive( Debug ) ]
pub struct Actor
{
  pub name : String,
}

#[ derive( Debug ) ]
pub struct Genre
{
  pub name : String,
}

#[ derive( Debug ) ]
pub struct Film
{
  pub title : String,
  pub rate : i32,
  pub actors : Vec< Actor >,
  pub genres : Vec< Genre >,
}

#[ derive( Clone ) ]
enum Values
{
  String( String ),
  Number( i32 ),
  List( Vec< Values > ),
}

impl Values
{
  fn str( &self ) -> Option< String > { if let Values::String( s ) = self { Some( s.to_owned() ) } else { None } }
  fn num( &self ) -> Option< i32 > { if let Values::Number( n ) = self { Some( n.to_owned() ) } else { None } }
  fn list( &self ) -> Option< Vec< Values > > { if let Values::List( l ) = self { Some( l.to_vec() ) } else { None } }
}

#[ extend_args( KWARGS( Values ) ) ]
fn new_film( title : String ) -> Film
{
  // * Problems with types. I think it can be better
  Film
  {
    title,
    rate : kwargs.get( "rate" ).and_then( | n | n.num() ).unwrap_or_default(),
    // 1. if value exist
    actors : kwargs.get( "actors" )
    // 2. if value needed type
    .and_then( | a | a.list()
    // 3. iterate over vector of Values
    .and_then( | v | Some( v.iter()
    // 4. map over this Values and try to convert it to needed type
    .map( | v | Actor{ name : v.str().unwrap_or_default() } ) 
    // 5. collect all Actors to vector
    .collect::< Vec< _ > >())
    )).unwrap_or_default(),
    genres : kwargs.get( "genres" )
    .and_then( | g | g.list()
    .and_then( | v | Some(v.iter()
    .map( | v | Genre{ name : v.str().unwrap_or_default() } ) 
    .collect::< Vec< _ > >())
    )).unwrap_or_default(),
  }
}

fn main()
{
  let f = new_film!
  (
    "Some film".to_owned(),
    actors = Values::List( vec![ Values::String( "Actor name".to_owned() ) ] )
  );
  dbg!( f );
  let f = new_film!
  (
    "Some second film".to_owned(),
    actors = Values::List( vec![ Values::String( "Actor name".to_owned() ) ] ),
    genres = Values::List( vec![ Values::String( "Horror".to_owned() ), Values::String( "Action".to_owned() ) ] ),
    rate = Values::Number( 100 )
  );
  dbg!( f );
}