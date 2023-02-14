import {useState} from "react";

interface Props {

}

export const LeaderBoard = (props: Props) => {
  const [state, setState] = useState([
      {position: 1, rocketData: 'asdfasfasfdasdf', data: 'rocketData asdfasfdasfasfasdfasdfa'},
      {position: 2, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 3, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 4, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 5, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 6, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 7, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 8, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 9, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
      {position: 10, rocketData: 'asdfasfasfdasdf', data: 'rocketData'},
  ])
  return (
    <div className="flex flex-col w-9/12 items-center bg-gray-300 w-1/2 mx-auto logs text-black border-blackk"
         style={{ height: '50vh', backgroundColor: 'rgb(173, 178, 175)' }}>
      <div className='flex flex-row w-full text-center'>
          <div className='border-2 border-black w-2/6'><span>Successful launches</span></div>
          <div className='border-2 border-black w-2/6'><span>Success rate</span></div>
          <div className='border-2 border-black w-2/6'><span>Revenue</span></div>
      </div>
      <div className='flex flex-col w-full text-center'>
          {state.map(rocket => {
                  return (
                      <div className='flex flex-row w-full text-center'>
                          <div className='w-2/6'><span>{rocket.position}</span></div>
                          <div className='w-2/6'><span>{rocket.rocketData}</span></div>
                          <div className='w-2/6'><span>{rocket.data}</span></div>
                      </div>
                  )})}
      </div>
    </div>
  );
};
