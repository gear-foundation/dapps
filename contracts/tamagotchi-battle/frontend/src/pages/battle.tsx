import { BattlePlayersQueue } from 'components/sections/battle-players-queue';
import { BattleWaitRegistration } from 'components/sections/battle-wait-registration';
import { useApp, useBattle } from 'app/context';
import { BattleWaitAdmin } from 'components/sections/battle-wait-admin';
import { BattleRound } from 'components/sections/battle-round';
import { BattleWinner } from 'components/sections/battle-winner';
// import { AnimatePresence, motion } from 'framer-motion';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battle, rivals, currentPlayer } = useBattle();

  return (
    <>
      {battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {battle &&
        ['GameIsOn', 'WaitNextRound'].includes(battle.state) &&
        Object.values(battle.pairs).length > 0 &&
        rivals.length && <BattleRound />}
      {battle && battle.state === 'GameIsOver' && rivals.length && currentPlayer && <BattleWinner battle={battle} />}
      {battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}
    </>
  );
};

// <AnimatePresence key="battle" initial={false}>
//   {battle &&
//     ['GameIsOn', 'WaitNextRound'].includes(battle.state) &&
//     Object.values(battle.pairs).length > 0 &&
//     rivals.length > 0 && (
//       <motion.div
//         key="battle-in-game"
//         className="flex flex-col grow"
//         initial={{ opacity: 0 }}
//         exit={{ opacity: 0, transition: { duration: 1, delay: 2.5 } }}
//         animate={{ opacity: 1 }}>
//         <BattleRound />
//       </motion.div>
//     )}
//
//   {battle && battle.state === 'GameIsOver' && rivals.length > 0 && currentPlayer && (
//     <motion.div
//       key="battle-winner"
//       className="flex flex-col grow"
//       initial={{ opacity: 0 }}
//       exit={{ opacity: 0, transition: { duration: 1, delay: 1 } }}
//       animate={{ opacity: 1 }}
//       transition={{ delay: 3 }}>
//       <BattleWinner battle={battle} />
//     </motion.div>
//   )}
// </AnimatePresence>;
