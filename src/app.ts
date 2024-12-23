import express from 'express';
import dotenv from 'dotenv';
import TaskManager from './TaskManager';
import PriorityManager from './PriorityManager';

dotenv.config();

const app = express();
const serverPort = process.env.PORT || 3000;

app.use(express.json());

interface ApplicationState {
    taskList: any[];
    priorityList: any[];
}

const appState: ApplicationState = {
    taskList: [],
    priorityList: [],
};

const taskHandler = new TaskManager();
const priorityHandler = new PriorityManager();

app.post('/tasks', (req, res) => {
    const { task } = req.body;
    try {
        const addedTask = taskHandler.addTask(task);
        appState.taskList.push(addedTask);
        res.status(201).json(addedTask);
    } catch (error) {
        res.status(400).send(error.message);
    }
});

app.put('/tasks/:id/priority', (req, res) => {
    const taskId = req.params.id;
    const { priority } = req.body;

    try {
        priorityHandler.setTaskPriority(taskId, priority);
        const targetedTask = appState.taskList.find(t => t.id === taskId);
        if (targetedTask) {
            targetedTask.priority = priority;
            res.json(targetedTask);
        } else {
            res.status(404).send('Task not found');
        }
    } catch (error) {
        res.status(400).send(error.message);
    }
});

app.listen(serverPort, () => {
    console.log(`Server is running on port ${serverPort}`);
});