export interface IWatchMsg {
  streamId: string;
  signedMsg: string;
  encodedId: string;
}

export interface IBroadcastMsg {
  streamId: string;
}

export interface IStopBroadcastingMsg extends IBroadcastMsg {}

export interface IOfferMsg {
  userId: string;
  description: RTCSessionDescription;
  streamId: string;
}

export interface IStopWatchingMsg {
  streamId: string;
}

export interface IAnswerMsg {
  watcherId: string;
  description: RTCSessionDescription;
}

export interface ICandidateMsg {
  id: string;
  candidate: RTCIceCandidate;
}

export interface IErrorResponse {
  message: string;
}

export interface GetInfoForUserMsg {
  streamId: string;
}
