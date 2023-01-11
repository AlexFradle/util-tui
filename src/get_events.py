from __future__ import print_function

import datetime
import os.path
import sys

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError
import json

# If modifying these scopes, delete the file token.json.
SCOPES = ['https://www.googleapis.com/auth/calendar.readonly']


def main():
    creds = None
    # The file token.json stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', SCOPES)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                'credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open('token.json', 'w') as token:
            token.write(creds.to_json())

    try:
        year = int(sys.argv[1])
        month = int(sys.argv[2])
        num_of_days = int(sys.argv[3])
    except IndexError:
        print(json.dumps({"error": "args wrong"}))
        return
    try:
        service = build('calendar', 'v3', credentials=creds)
        start = datetime.datetime(year, month, 1).isoformat() + "Z"
        end = datetime.datetime(year, month, num_of_days, 23, 59, 59).isoformat() + "Z"
        events_result = service.events().list(calendarId='h7gph7lv2pj9e164pbqg3p2qd1u3ula9@import.calendar.google.com',
                                              timeMin=start, timeMax=end,
                                              singleEvents=True,
                                              orderBy='startTime').execute()
        events = events_result.get('items', [])
        data = [
            {
                "start": event['start'].get('dateTime', event['start'].get('date')),
                "end": event["end"].get("dateTime", event["end"].get("date")),
                "title": event["summary"],
                "description": event["description"],

            }
            for event in events
        ]
        print(json.dumps(data))

    except HttpError as error:
        print(json.dumps({"error": 'An error occurred: %s' % error}))


if __name__ == '__main__':
    main()
