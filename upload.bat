cargo lambda build --release --arm64
copy target\lambda\moveit-nie-istnieje\bootstrap .
copy generate\jokes.txt .
powershell Compress-Archive -Path bootstrap, generate\jokes.txt, web\ -DestinationPath lambda.zip
aws lambda update-function-code --function-name moveit-nie-istnieje --zip-file fileb://lambda.zip
del bootstrap lambda.zip jokes.txt