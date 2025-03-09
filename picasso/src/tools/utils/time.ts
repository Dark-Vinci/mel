export function getDayDifference(date1: Date, date2: Date): number {
    const msInDay = 1000 * 60 * 60 * 24;
    return Math.round((date2.getTime() - date1.getTime()) / msInDay);
}

export function generateRandomColor(): string {
    let result = '#';
    let db = '0123456789abcdef';

    for (let i = 0; i < 6; i++) {
        let rand = Math.random() * db.length;
        result += db[rand];
    }

    return result;
}
