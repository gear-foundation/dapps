import { useAtom } from 'jotai';
import { atomWithStorage } from 'jotai/utils';

const USER_NAME_KEY = 'user_name';
const userNameAtom = atomWithStorage(USER_NAME_KEY, 'Player');

export function useUserName() {
  const [userName, setUserName] = useAtom(userNameAtom);
  return { userName, setUserName };
}
