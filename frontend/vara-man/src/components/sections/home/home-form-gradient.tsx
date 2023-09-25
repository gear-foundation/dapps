type HomeFormGradientProps = BaseComponentProps & {}

export function HomeFormGradient({}: HomeFormGradientProps) {
  return (
    <svg
      className="w-full h-[84%]"
      width="656"
      height="278"
      viewBox="0 0 656 278"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M653 257.147V22.6471C653 11.6014 644.046 2.64709 633 2.64709H22.5C11.4543 2.64709 2.5 11.6014 2.5 22.6471V257.147"
        stroke="url(#paint0_linear_212_341010)"
        strokeWidth="4"
      />
      <rect
        x="650.5"
        y="277.647"
        width="148"
        height="5"
        transform="rotate(-90 650.5 277.647)"
        fill="url(#paint1_linear_212_341010)"
      />
      <rect
        x="0.499817"
        y="277.647"
        width="148"
        height="4"
        transform="rotate(-90 0.499817 277.647)"
        fill="url(#paint2_linear_212_341010)"
      />
      <defs>
        <linearGradient
          id="paint0_linear_212_341010"
          x1="640.5"
          y1="2.64709"
          x2="149.5"
          y2="296.147"
          gradientUnits="userSpaceOnUse"
        >
          <stop stopColor="#2F81ED" />
          <stop offset="1" stopColor="#2BD071" />
        </linearGradient>
        <linearGradient
          id="paint1_linear_212_341010"
          x1="650.5"
          y1="279.835"
          x2="684.306"
          y2="217.693"
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0.657594" stopColor="#1F1F1F" />
          <stop offset="1" stopColor="#202020" stopOpacity="0" />
        </linearGradient>
        <linearGradient
          id="paint2_linear_212_341010"
          x1="0.499817"
          y1="279.397"
          x2="24.0741"
          y2="225.231"
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0.657594" stopColor="#1F1F1F" />
          <stop offset="1" stopColor="#202020" stopOpacity="0" />
        </linearGradient>
      </defs>
    </svg>
  )
}
