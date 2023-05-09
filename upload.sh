cargo lambda build --release --arm64
cp target/lambda/moveit-nie-istnieje/bootstrap .
cp generate/jokes.txt .
zip lambda.zip bootstrap jokes.txt web/ -r
aws lambda update-function-code --function-name moveit-nie-istnieje --zip-file fileb://lambda.zip
rm bootstrap lambda.zip jokes.txt