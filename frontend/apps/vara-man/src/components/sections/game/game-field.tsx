type GameFieldProps = {};

export function GameField({}: GameFieldProps) {
  return (
    <div>
      <canvas id="canvas" className="w-full bg-neutral-600 aspect-[110/72]" />
    </div>
  );
}
