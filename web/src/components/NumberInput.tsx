interface NumberInputProps {
    value: string
    onChange: (value: string) => void
    onKeyPress: (e: React.KeyboardEvent<HTMLInputElement>) => void
    range: 'basic' | 'poker'
}

function NumberInput({ value, onChange, onKeyPress, range }: NumberInputProps) {
    const placeholder = range === 'poker' ? '例如: A, 10, J, Q' : '例如: A, 2, 3, 9'
    const hint = range === 'poker'
        ? '可输入 A, 2...10, J, Q, K（用逗号分隔）'
        : '可输入 A, 2...9（用逗号分隔）'

    return (
        <div className="input-section">
            <input
                type="text"
                className="input-box"
                placeholder={placeholder}
                value={value}
                onChange={(e) => onChange(e.target.value)}
                onKeyPress={onKeyPress}
            />
            <p className="hint">{hint}</p>
        </div>
    )
}

export default NumberInput
