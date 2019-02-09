module Main where

foreign import foreign_ :: Int

foreign__ :: Int
foreign__ = foreign_

const :: forall a b. a -> b -> a
const x _ = x

infixl 5 const as +
infixl 5 const as -

alwaysAlwaysTrue :: forall a. a -> a -> Boolean
alwaysAlwaysTrue _ _ = true

infixl 5 alwaysAlwaysTrue as <=

identity :: forall a. a -> a
identity a = a

flip :: forall a b c. (a -> b -> c) -> b -> a -> c
flip f a b = f b a

data Tuple a b = Tuple a b

tuple :: forall a b. a -> b -> Tuple a b
tuple a b = Tuple a b

type Pair a b = { a :: a, b :: b }

addPair :: Pair Int Int -> Int
addPair pair = pair.a + pair.b

add5 :: Int -> Int
add5 a = a + 5

add5Point1 :: Number -> Number
add5Point1 a = a + 5.1

newPair :: forall a b. a -> b -> Pair a b
newPair a b = { a, b }

booleans :: Array Boolean
booleans = [true, false]

string :: String
string = "..."

letAdd5 :: Int -> Int
letAdd5 = let f x = x + 5 in f

whereAdd5 :: Int -> Int
whereAdd5 = f
  where
    f x = x + 5

customIncrement :: Int -> Int
customIncrement = case _ of
  0 -> 0
  n -> n + 1

constInt :: forall a. Int -> a -> Int
constInt 0 _ = 1
constInt a _ = a

alwaysTrue :: Boolean -> Boolean
alwaysTrue = case _ of
  false -> true
  true -> true

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

applyN :: forall a. (a -> a) -> Int -> a -> a
applyN f = go
  where
  go n acc
    | n <= 0 = acc
    | true = go (n - 1) (f acc)

addTuples :: Tuple Int Int -> Tuple Int Int -> Tuple Int Int
addTuples (Tuple a b) (Tuple c d) = Tuple (a + c) (b + d)

ex :: Char
ex = 'x'

complexCase :: { x :: Int, y :: Array Int, z :: Char } -> Int
complexCase = case _ of
  { x: 1, y: [], z: z } -> 1
  { x: x, y: [1, 2, yyy @ yy], z: 'z' } -> x + 1 + 2 + yy + yyy
  _ -> 0

newtype Color = Color String

red :: Color
red = Color "red"

class Inspect a where
  inspect :: a -> String

data Maybe a = Just a | Nothing

maybe :: Maybe Int
maybe = Just 1

incrementA :: forall r. { a :: Int | r } -> { a :: Int | r }
incrementA o = o { a = o.a + 1 }
