export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;
export type JSONArray = Array<JSONValue>;
export type JSONObject = { [x: string]: JSONValue };

export type LogFunction = (service: string, message: string, level: LogLevel) => void;

export enum LogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Trace = 4,
    Debug = 5,
}
