import {Meeting} from "../models";

export type MeetingCallback = ((meeting: Meeting) => void) | null;