import express from 'express';
import dotenv from 'dotenv';
import TaskManager from './TaskManager';
import PriorityManager from './PriorityManager';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

interface AppState {
    tasks: any[];
    priorities: any[];
}

const state: AppState = {
    tasks: [],
    priorities: [],
};

const taskManager = new TaskManager();

const priorityManager = new PriorityManager();

app.post('/tasks', (req, res) => {
    const { task } = req.body;
    try {
        const newTask = taskManager.addTask(task);
        state.tasks.push(newTask);
        res.status(201).json(newTask);
    } catch (error) {
        res.status(400).send(error.message);
    }
});

app.put('/tasks/:id/priority', (req, res) => {
    const { id } = req.params;
    const { priority } = req.body;

    try {
        priorityManager.setPriority(id, priority);
        const task = state.tasks.find(t => t.id === id);
        if (task) {
            task.priority = priority;
            res.json(task);
        } else {
            res.status(404).send('Task not found');
        }
    } catch (error) {
        res.status(400).send(error.message);
    }
});

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});