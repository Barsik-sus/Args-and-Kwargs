use extend_fn_args::*;

#[ extend_args( ARGS( i32 ), KWARGS( f32 ) ) ]
fn sum( first_arg : i32 ) -> f32
{
  first_arg as f32
  +
  args.iter().fold( 0.0, | acc, val | acc + *val as f32 )
  +
  kwargs.iter().fold( 0.0, | acc, val | acc + *val.1)
}

#[ extend_args( ARGS( i32 ) ) ]
fn sum_without_kwargs( first_arg : i32 ) -> f32
{
  first_arg as f32
  +
  args.iter().fold( 0.0, | acc, val | acc + *val as f32 )
}

#[ extend_args( KWARGS( f32 ) ) ]
fn sum_without_args( first_arg : i32 ) -> f32
{
  first_arg as f32
  +
  kwargs.iter().fold( 0.0, | acc, val | acc + *val.1)
}

#[ test ]
fn with_args_and_kwargs()
{
  assert_eq!
  (
    sum!( 42 ),
    42.0
  );
  assert_eq!
  (
    sum!( 42, a = 10.0 + 4.0, b = 14.2 ),
    42.0 + 10.0 + 4.0 + 14.2
  );
  assert_eq!
  (
    sum!( 42, 12 + 4, 14 ),
    42.0 + 12.0 + 4.0 + 14.0
  );
  assert_eq!
  (
    sum!( 42, 12 + 4, 14 ; c = 12.0 + 4.0, d = 85.0 - 10.0 ),
    42.0 + 12.0 + 4.0 + 14.0 + 12.0 + 4.0 + 85.0 - 10.0
  );
  assert_eq!
  (
    sum!( 32, 3 ),
    32.0 + 3.0
  );
  let v = &mut vec![ 100, 24 ];
  assert_eq!
  (
    // sum!( 32, k = 13 => v ), // ! Don't work
    // 32.0 + 13.0 + 100.0 + 24.0
    sum!( 32 => v ),
    32.0 + 100.0 + 24.0
  );
  let hm = &mut HashMap::from([( "qwe".to_owned(), 228.8 )]);
  assert_eq!
  (
    // sum!( 32, 14  => ; hm ), // ! Don't work
    // 32.0 + 14.0 + 228.8
    sum!( 32 => ; hm ),
    32.0 + 228.8
  );
  let v = &mut vec![ 100, 24 ];
  let hm = &mut HashMap::from([( "qwe".to_owned(), 228.8 )]);
  assert_eq!
  (
    // sum!( 32, 14 => v ; hm ), // ! Don't work
    // 32.0 + 14.0 + 228.8 + 100.0 + 24.0
    sum!( 32 => v ; hm ),
    32.0 + 228.8 + 100.0 + 24.0
  );
}

#[ test ]
fn with_args()
{
  assert_eq!
  (
    sum_without_kwargs!( 42 ),
    42.0
  );
  assert_eq!
  (
    sum_without_kwargs!( 42, 12 + 4, 14 ),
    42.0 + 12.0 + 4.0 + 14.0
  );

  assert_eq!
  (
    sum_without_kwargs!( 7 => &mut vec![ 1, 2, 3 ] ),
    7.0 + 1.0 + 2.0 + 3.0
  )
}

#[ test ]
fn with_kwargs()
{
  assert_eq!
  (
    sum_without_args!( 42 ),
    42.0
  );
  assert_eq!
  (
    sum_without_args!( 42, a = 10.0 + 4.0, b = 14.2 ),
    42.0 + 10.0 + 4.0 + 14.2
  );
  let hm = &mut HashMap::from([( "qwe".to_owned(), 228.8 )]);
  assert_eq!
  (
    sum_without_args!( 32 => hm ),
    260.8
  );
}