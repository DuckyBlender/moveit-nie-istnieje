import os
import json
import requests
from dotenv import load_dotenv

MODEL = "gpt-3.5-turbo"
PROMPT = "Create a short and witty diss in Polish about a fictional public transport startup called Moveit. Moveit is an application that notifies a user when a bus is late, that is all. Ensure it's only one sentence and effectively mocks the non-existent company. Make sure to write it only in Polish. For example:\nTwój tata jest jak Moveit - nie istnieje. Use this format or similar."
AMOUNT = 3

load_dotenv()

token = os.getenv("OPENAI_TOKEN")
if token is None:
    raise ValueError("OPENAI_TOKEN not set in .env")

headers = {
    "Content-Type": "application/json",
    "Authorization": f"Bearer {token}",
}


def send_request(token, request):
    response = requests.post(
        "https://api.openai.com/v1/chat/completions",
        headers=headers,
        data=json.dumps(request),
    )

    if response.status_code != 200:
        print(f"Error sending request: {response.text}")
        return None

    res = response.json()
    content = res["choices"][0]["message"]["content"]

    if content == "null":
        return None

    return content


request = {
    "model": MODEL,
    "max_tokens": 150,
    "temperature": 1.0,
    "top_p": 0.8,
    "messages": [
        {
            "role": "system",
            "content": PROMPT,
        }
    ],
}

print(f"Sending {AMOUNT} requests to GPT")

jokes = []
completed = 0
failed = 0

for i in range(AMOUNT):
    joke = send_request(token, request)
    if joke is not None:
        print(f"#{i + 1}: {joke}")
        completed += 1
        jokes.append(joke)
    else:
        print(f"#{i + 1}: Failed")
        failed += 1

with open("joke.txt", "w") as file:
    for joke in jokes:
        file.write(joke + "\n")

print(f"Saved {completed} jokes to file")
print(f"Failed to save {failed} jokes to file")

print("Done!")
