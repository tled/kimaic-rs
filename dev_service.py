import json
from flask import Flask, request


app = Flask(__name__)


@app.route("/api/timesheets", methods=['GET'])
def timesheets():
    print(f"X-AUTH-USER: {request.headers.get('X-AUTH-USER')}")
    print(f"X-AUTH-TOKEN: {request.headers.get('X-AUTH-TOKEN')}")
    if request.args.get("page") and int(request.args.get("page")) > 1:
        return '{"code":404,"message":"Not Found"}', 404
    else:
        with open('timesheet_api_test_output', "r") as f:
            return f.read()


if __name__ == "__main__":
    app.run()
