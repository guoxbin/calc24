interface OperatorSelectorProps {
    operators: string[]
    onChange: (operators: string[]) => void
}

interface OperatorOption {
    value: string
    label: string
}

function OperatorSelector({ operators, onChange }: OperatorSelectorProps) {
    const allOperators: OperatorOption[] = [
        { value: '+', label: '+' },
        { value: '-', label: '-' },
        { value: '*', label: '*' },
        { value: '/', label: '/' },
        { value: 'factorial', label: '阶乘' },
        { value: 'pow', label: '乘方' },
        { value: 'sqrt', label: '开方' },
        { value: 'log', label: 'log' },
    ]

    const handleToggle = (value: string) => {
        if (operators.includes(value)) {
            onChange(operators.filter(op => op !== value))
        } else {
            onChange([...operators, value])
        }
    }

    return (
        <div className="range-options">
            <span className="range-label">计算符号：</span>
            <div className="operator-group">
                {allOperators.map(({ value, label }) => (
                    <div key={value} className="radio-option">
                        <input
                            type="checkbox"
                            id={`op-${value}`}
                            name="operators"
                            value={value}
                            checked={operators.includes(value)}
                            onChange={() => handleToggle(value)}
                        />
                        <label htmlFor={`op-${value}`}>{label}</label>
                    </div>
                ))}
            </div>
        </div>
    )
}

export default OperatorSelector
