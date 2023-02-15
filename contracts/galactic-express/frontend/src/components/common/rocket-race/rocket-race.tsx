import { RacePosition } from "../../../pages/launch";
import { SessionStatus } from "../../../app/types/battles";

interface Props extends RacePosition {
    sessionStatus: SessionStatus
}

export const RocketRace = (props: Props) => {
    const { xoffset = 10, bgColor = '#ADB2AF', id, sessionStatus, payload, alive } = props;

    function handleAddressBySessionStatus(sessionStatus: SessionStatus): string {
        if (sessionStatus === SessionStatus.REGISTRATION) {
            return `${id.slice(1, 5)}...${id.slice(-5)}`
        }

        return ''
    }

    return (
        <div className="h-1/4 w-full border-2 border-b-gray-900"
            key={id}
            style={{ background: alive ? bgColor : '#7b0015'}}>
            <span className="text-h2" style={{
                position: "absolute",
                left: `${xoffset}%`,
            }}>{`${!alive ? '🔥' : '🚀'}`}</span><span className="players-ready"> {`${handleAddressBySessionStatus(sessionStatus)}`}</span>
        </div>
    )
}
