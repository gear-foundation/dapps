import { ActorId } from 'sails-js';

declare global {
  export interface ContractInfo {
    admins: Array<ActorId>;
    program_id: ActorId;
    registration_time: string;
  }
}
