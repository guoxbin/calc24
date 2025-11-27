interface RangeSelectorProps {
    range: 'basic' | 'poker'
    onChange: (range: 'basic' | 'poker') => void
}

function RangeSelector({ range, onChange }: RangeSelectorProps) {
    return (
        <div className="range-options">
            <span className="range-label">数字范围：</span>
            <div className="radio-group">
                <div className="radio-option">
                    <input
                        type="radio"
                        id="range-basic"
                        name="range"
                        value="basic"
                        checked={range === 'basic'}
                        onChange={(e) => onChange(e.target.value as 'basic' | 'poker')}
                    />
                    <label htmlFor="range-basic">A, 2, 3, 4, 5, 6, 7, 8, 9（A代表1）</label>
                </div>
                <div className="radio-option">
                    <input
                        type="radio"
                        id="range-poker"
                        name="range"
                        value="poker"
                        checked={range === 'poker'}
                        onChange={(e) => onChange(e.target.value as 'basic' | 'poker')}
                    />
                    <label htmlFor="range-poker">A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K（A, J, Q, K代表1, 11, 12, 13）</label>
                </div>
            </div>
        </div>
    )
}

export default RangeSelector
