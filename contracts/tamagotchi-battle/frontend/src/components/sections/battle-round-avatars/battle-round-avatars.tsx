export const BattleRoundAvatars = () => {
  return (
    <div className="relative grow grid grid-cols-[repeat(2,minmax(auto,445px))] justify-between gap-10 mt-10 xl:mt-15">
      {/*<div className="w-full h-full flex flex-col">*/}
      {/*  <TamagotchiAvatar*/}
      {/*    inBattle*/}
      {/*    age={getTamagotchiAgeDiff(warriors[0].dateOfBirth)}*/}
      {/*    hasItem={[]}*/}
      {/*    energy={warriors[1]?.energy}*/}
      {/*    className="grow w-full h-full "*/}
      {/*    isActive={battle?.currentTurn === 0 && battle?.state !== 'GameIsOver'}*/}
      {/*    isWinner={battle?.state === 'GameIsOver' && battle.winner === battle.players[0].tmgId}*/}
      {/*    isDead={battle?.state === 'GameIsOver' && battle.winner !== battle.players[0].tmgId}*/}
      {/*  />*/}
      {/*</div>*/}
      {/*<div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-8">*/}
      {/*  /!*{winner && (*!/*/}
      {/*  /!*  <p className="flex flex-col items-center">*!/*/}
      {/*  /!*    <strong className="text-2xl leading-normal xl:typo-h2 text-primary truncate max-w-[9ch]">*!/*/}
      {/*  /!*      {winner.name}*!/*/}
      {/*  /!*    </strong>*!/*/}
      {/*  /!*    <span className="text-[60px] leading-[1.2] font-bold xl:typo-h1">Win</span>*!/*/}
      {/*  /!*  </p>*!/*/}
      {/*  /!*)}*!/*/}

      {/*  <button*/}
      {/*    className={clsx(*/}
      {/*      'btn items-center gap-2 min-w-[250px]',*/}
      {/*      battle?.state === 'Moves'*/}
      {/*        ? 'bg-error text-white hover:bg-red-600 transition-colors'*/}
      {/*        : battle?.state === 'GameIsOver'*/}
      {/*        ? buttonStyles.secondary*/}
      {/*        : buttonStyles.primary,*/}
      {/*      buttonStyles.button,*/}
      {/*    )}*/}
      {/*    onClick={handleAttack}*/}
      {/*    disabled={isPending}>*/}
      {/*    <Icon name="swords" className="w-5 h-5" />{' '}*/}
      {/*    {battle?.state === 'Moves'*/}
      {/*      ? 'Attack'*/}
      {/*      : battle?.state === 'Waiting'*/}
      {/*      ? 'Wait...'*/}
      {/*      : battle?.state === 'GameIsOver'*/}
      {/*      ? 'Finish Game'*/}
      {/*      : ''}*/}
      {/*  </button>*/}
      {/*</div>*/}
      {/*<div className="w-full h-full flex flex-col">*/}
      {/*  <TamagotchiAvatar*/}
      {/*    inBattle*/}
      {/*    age={getTamagotchiAgeDiff(warriors[1].dateOfBirth)}*/}
      {/*    hasItem={getAttributesById(store, battle.players[1].attributes)}*/}
      {/*    energy={warriors[0].energy}*/}
      {/*    className="grow w-full h-full "*/}
      {/*    isActive={battle?.currentTurn === 1 && battle?.state !== 'GameIsOver'}*/}
      {/*    isWinner={battle?.state === 'GameIsOver' && battle.winner === battle.players[1].tmgId}*/}
      {/*    isDead={battle?.state === 'GameIsOver' && battle.winner !== battle.players[1].tmgId}*/}
      {/*  />*/}
      {/*</div>*/}
    </div>
  );
};
