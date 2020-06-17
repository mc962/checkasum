export class Payload {
    public filePath: string;
    public checksum: string;

    constructor(filePath: string | undefined, checksum: string | undefined) {
        this.filePath = filePath || '';
        this.checksum = checksum || '';
    }

    public static parse(properties: Payload) {
        return new Payload(
            properties.filePath,
            properties.checksum
        )
    }

    public isPresent() {
        return [
            this.filePath,
            this.checksum
        ].every((item) => !!item);
    }
}


export enum Status {
    Success,
    Failure,
}

export interface Result {
    status: Status;
    filePath: string;
    checksum: string;
    errors?: Array<HashingError>
}

export interface HashingError {
    message: string;
}