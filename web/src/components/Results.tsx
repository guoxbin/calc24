interface ResultsProps {
    results: {
        solutions?: string[]
        error?: string
    } | null
}

function Results({ results }: ResultsProps) {
    if (!results) {
        return null
    }

    return (
        <div className="result-section show">
            {results.error ? (
                <p className="error">{results.error}</p>
            ) : results.solutions && results.solutions.length > 0 ? (
                <>
                    <div className="result-title">找到以下解法：</div>
                    {results.solutions.map((solution, index) => (
                        <div
                            key={index}
                            className="solution"
                            dangerouslySetInnerHTML={{ __html: `${index + 1}. ${solution}` }}
                        />
                    ))}
                </>
            ) : (
                <p className="no-solution">无解！这4个数字无法算出24。</p>
            )}
        </div>
    )
}

export default Results
