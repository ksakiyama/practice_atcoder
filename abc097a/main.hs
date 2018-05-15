main = do
  [a,b,c,d] <- map read . words <$> getLine :: IO [Int]
  if (abs $ a - c) <= d || (abs $ a - b) <=d && (abs $ b - c) <=d
    then putStrLn "Yes"
    else putStrLn "No"

-- main :: IO ()
-- main = do
--     [a,b,c,d] <- map read . words <$> getLine
--     putStrLn $ colorfulTransceivers a b c d
  
-- colorfulTransceivers :: Int -> Int -> Int -> Int -> String
-- colorfulTransceivers a b c d = 
--     if ( abs(a-c) <= d || abs (a-b) <= d && abs(b-c) <= d)
--         then "Yes"
--         else "No"
--     where

-- main = do
--   [a,_,_,_] <- map read . words <$> getLine :: IO [Int]
--   print a
  -- if (abs $ a - b) <= d && (abs $ b - c) <= d || (abs $ a - c) <= d
  --   then putStrLn "Yes"
  --   else putStrLn "No"