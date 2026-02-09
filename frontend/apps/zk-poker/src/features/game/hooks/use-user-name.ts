import { useAtom } from 'jotai';
import { atomWithStorage } from 'jotai/utils';

const USER_NAME_KEY = 'zkpoker_user_name';
const userNameAtom = atomWithStorage(USER_NAME_KEY, '');

export function useUserName() {
  const [userName, setUserName] = useAtom(userNameAtom);

  return { userName: userName || 'Player', setUserName, isUserNameSet: !!userName };
}
