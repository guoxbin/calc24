import { useState } from 'react'
import './App.css'
import RangeSelector from './components/RangeSelector'
import OperatorSelector from './components/OperatorSelector'
import NumberInput from './components/NumberInput'
import Results from './components/Results'

interface CalculateResponse {
    solutions?: string[]
    error?: string
}

interface GenerateResponse {
    problem?: string
}

function App() {
    const [range, setRange] = useState<'basic' | 'poker'>('basic')
    const [operators, setOperators] = useState<string[]>(['+', '-', '*', '/'])
    const [numbers, setNumbers] = useState<string>('')
    const [results, setResults] = useState<CalculateResponse | null>(null)
    const [isCalculating, setIsCalculating] = useState<boolean>(false)
    const [isGenerating, setIsGenerating] = useState<boolean>(false)

    const handleCalculate = async () => {
        const numbersStr = numbers.trim().toUpperCase()
        const tokens = numbersStr.split(/[,，]+/).map(s => s.trim()).filter(s => s.length > 0)

        // 验证输入数量
        if (tokens.length !== 4) {
            setResults({ error: '请输入4个数字，用逗号分隔！' })
            return
        }

        // 验证运算符
        if (operators.length === 0) {
            setResults({ error: '请至少选择一个计算符号！' })
            return
        }

        // 根据选择的范围验证输入
        if (range === 'poker') {
            const valid = tokens.every(t => /^(A|10|[2-9]|J|Q|K)$/.test(t))
            if (!valid) {
                setResults({ error: '只能输入 A, 2-10, J, Q, K！' })
                return
            }
        } else {
            const valid = tokens.every(t => /^[A2-9]$/.test(t))
            if (!valid) {
                setResults({ error: '只能输入 A, 2-9！' })
                return
            }
        }

        setIsCalculating(true)
        try {
            const response = await fetch('/calculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    numbers: tokens.join(','),
                    range: range,
                    operators: operators
                })
            })

            const data: CalculateResponse = await response.json()
            setResults(data)
        } catch (error) {
            setResults({ error: '计算出错，请重试！' })
        } finally {
            setIsCalculating(false)
        }
    }

    const handleGenerate = async () => {
        if (operators.length === 0) {
            setResults({ error: '请至少选择一个计算符号！' })
            return
        }

        setIsGenerating(true)
        try {
            const response = await fetch('/generate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    range: range,
                    operators: operators
                })
            })

            const data: GenerateResponse = await response.json()
            if (data.problem) {
                setNumbers(data.problem)
                setResults(null)
            }
        } catch (error) {
            console.error('Generate error:', error)
        } finally {
            setIsGenerating(false)
        }
    }

    const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            handleCalculate()
        }
    }

    return (
        <div className="container">
            <h1>算24点</h1>
            <p className="subtitle">输入4个数字，计算如何得到24</p>

            <RangeSelector range={range} onChange={setRange} />
            <OperatorSelector operators={operators} onChange={setOperators} />

            <NumberInput
                value={numbers}
                onChange={setNumbers}
                onKeyPress={handleKeyPress}
                range={range}
            />

            <button
                className="calc-button"
                onClick={handleCalculate}
                disabled={isCalculating}
            >
                {isCalculating ? '计算中...' : '算'}
            </button>

            <button
                className="generate-btn"
                onClick={handleGenerate}
                disabled={isGenerating}
            >
                {isGenerating ? '...' : '出题'}
            </button>

            <Results results={results} />
        </div>
    )
}

export default App
