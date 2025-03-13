import dotenv from 'dotenv';
dotenv.config();

interface Task {
    id: number;
    title: string;
    completed: boolean;
}

let tasks: Task[] = [];

function addTask(title: string): Task {
    const task: Task = {
        id: tasks.length + 1,
        title: title,
        completed: false
    };
    tasks.push(task);
    return task;
}

function removeTask(id: number): boolean {
    const initialLength = tasks.length;
    tasks = tasks.filter(task => task.id !== id);
    return tasks.length < initialLength;
}

function getTasks(): Task[] {
    return tasks;
}

function markAsCompleted(id: number): boolean {
    const task = tasks.find(task => task.id === id);
    if (task) {
        task.completed = true;
        return true;
    }
    return false;
}

export { addTask, removeTask, getTasks, markAsCompleted };