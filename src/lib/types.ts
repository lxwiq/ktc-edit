export type Primitive = string | number | boolean | null;
export type JSONValue = Primitive | { [key: string]: JSONValue } | JSONValue[];
