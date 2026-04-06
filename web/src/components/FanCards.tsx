import './FanCards.css'

interface Card {
  rank: string
  suit: string
  angle: number
  zIndex: number
}

const cards: Card[] = [
  { rank: 'K', suit: '♠', angle: -55, zIndex: 1 },
  { rank: 'Q', suit: '♠', angle: -48, zIndex: 2 },
  { rank: 'J', suit: '♠', angle: -41, zIndex: 3 },
  { rank: '10', suit: '♥', angle: -34, zIndex: 4 },
  { rank: '9', suit: '♥', angle: -27, zIndex: 5 },
  { rank: '8', suit: '♥', angle: -20, zIndex: 6 },
  { rank: 'A', suit: '♦', angle: -13, zIndex: 7 },
  { rank: '2', suit: '♦', angle: -6, zIndex: 8 },
  { rank: '3', suit: '♦', angle: 1, zIndex: 9 },
  { rank: '4', suit: '♣', angle: 8, zIndex: 10 },
  { rank: '5', suit: '♣', angle: 15, zIndex: 11 },
  { rank: '6', suit: '♣', angle: 22, zIndex: 12 },
  { rank: '7', suit: '♣', angle: 29, zIndex: 13 },
  { rank: '8', suit: '♣', angle: 36, zIndex: 14 },
  { rank: '9', suit: '♣', angle: 43, zIndex: 15 },
  { rank: '10', suit: '♠', angle: 50, zIndex: 16 },
  { rank: 'J', suit: '♥', angle: 57, zIndex: 17 },
]

function PokerCard({ card }: { card: Card }) {
  const isRed = card.suit === '♥' || card.suit === '♦'
  const color = isRed ? '#dc2626' : '#1f2937'

  return (
    <div
      className="poker-card"
      style={{
        transform: `rotate(${card.angle}deg)`,
        zIndex: card.zIndex,
      }}
    >
      <svg
        viewBox="0 0 450 650"
        className="card-svg"
        style={{ filter: 'drop-shadow(0 10px 20px rgba(0,0,0,0.3))' }}
      >
        {/* Card background */}
        <rect
          x="5"
          y="5"
          width="440"
          height="640"
          rx="25"
          fill="#ffffff"
          stroke="#e5e7eb"
          strokeWidth="5"
        />

        {/* Top left rank and suit */}
        <text
          x="40"
          y="90"
          fontSize="60"
          fontWeight="bold"
          fill={color}
          fontFamily="Arial, sans-serif"
        >
          {card.rank}
        </text>
        <text
          x="40"
          y="160"
          fontSize="60"
          fill={color}
        >
          {card.suit}
        </text>

        {/* Center large suit */}
        <text
          x="225"
          y="375"
          fontSize="180"
          textAnchor="middle"
          fill={color}
          style={{ opacity: 0.9 }}
        >
          {card.suit}
        </text>

        {/* Bottom right rank and suit (rotated) */}
        <g transform="rotate(180, 225, 325) translate(0, -20)">
          <text
            x="40"
            y="90"
            fontSize="60"
            fontWeight="bold"
            fill={color}
            fontFamily="Arial, sans-serif"
          >
            {card.rank}
          </text>
          <text
            x="40"
            y="160"
            fontSize="60"
            fill={color}
          >
            {card.suit}
          </text>
        </g>
      </svg>
    </div>
  )
}

export default function FanCards() {
  return (
    <div className="fan-container">
      <div className="fan-cards-wrapper">
        {cards.map((card, index) => (
          <PokerCard key={index} card={card} />
        ))}
      </div>
    </div>
  )
}
