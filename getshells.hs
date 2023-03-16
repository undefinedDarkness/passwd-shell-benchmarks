import System.IO
import Text.Printf
import Data.List (sort, group)

infixl 0 |>
(|>) :: a -> (a -> b) -> b
x |> f = f x

lastColumn :: String -> String
lastColumn s
    | ':' `elem` s  = lastColumn $ drop 1 $ dropWhile (/=':') s
    | otherwise     = s

prettyPrint :: [(String, Int)] -> String
prettyPrint [] = ""
prettyPrint [(k,v)] = printf "%s : %v\n" k v
prettyPrint (x@(k,v):xs) = prettyPrint [x] ++ prettyPrint xs

main :: IO ()
main = do
    entries <- readFile "passwd"

    let counts = lines entries
            |> fmap lastColumn
            |> sort
            |> group
            |> fmap (\g -> (head g, length g))

    putStr $ prettyPrint counts

