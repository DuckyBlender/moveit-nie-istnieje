cargo lambda build --release --arm64
cp target/lambda/moveit-nie-istnieje/bootstrap .
zip lambda.zip bootstrap jokes.txt template.html
aws lambda update-function-code --function-name moveit-nie-istnieje --zip-file fileb://lambda.zip