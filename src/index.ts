import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import rateLimit from 'express-rate-limit';
import { body, validationResult } from 'express-validator';
import dotenv from 'dotenv';
import sqlite3 from 'sqlite3';
import { open } from 'sqlite';

// Load environment variables
dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// SQLite database setup
let db: any;
(async () => {
  db = await open({
    filename: './contact_messages.db',
    driver: sqlite3.Database
  });
  await db.run(`CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    country TEXT NOT NULL,
    phone TEXT,
    company TEXT,
    message TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
  )`);
})();

// Middlewares
app.use(express.json());
app.use(cors());
app.use(helmet());
app.use(rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // limit each IP to 100 requests per windowMs
}));

// Contact form endpoint
app.post('/contact', [
  body('name').trim().notEmpty().withMessage('Name is required'),
  body('email').isEmail().withMessage('Valid email is required'),
  body('country').trim().notEmpty().withMessage('Country/Region is required'),
  body('phone').optional().isMobilePhone('any').withMessage('Phone number is invalid'),
  body('company').optional().trim(),
  body('message').trim().notEmpty().withMessage('Message is required'),
], async (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    return res.status(400).json({ errors: errors.array() });
  }

  const { name, email, country, phone, company, message } = req.body;
  try {
    await db.run(
      `INSERT INTO messages (name, email, country, phone, company, message) VALUES (?, ?, ?, ?, ?, ?)`,
      [name, email, country, phone || null, company || null, message]
    );
    res.status(200).json({ message: 'Contact form submitted and saved successfully.' });
  } catch (err) {
    res.status(500).json({ error: 'Failed to save message.' });
  }
});

// Endpoint pour récupérer tous les messages (admin)
app.get('/messages', async (req, res) => {
  try {
    const messages = await db.all('SELECT * FROM messages ORDER BY created_at DESC');
    res.status(200).json(messages);
  } catch (err) {
    res.status(500).json({ error: 'Failed to fetch messages.' });
  }
});

// Endpoint pour supprimer un message par id
app.delete('/messages/:id', async (req, res) => {
  const { id } = req.params;
  try {
    const result = await db.run('DELETE FROM messages WHERE id = ?', id);
    if (result.changes > 0) {
      res.status(200).json({ message: 'Message deleted successfully.' });
    } else {
      res.status(404).json({ error: 'Message not found.' });
    }
  } catch (err) {
    res.status(500).json({ error: 'Failed to delete message.' });
  }
});

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
