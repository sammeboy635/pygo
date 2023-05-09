import os
import sys
from PyQt5.QtWidgets import QApplication, QMainWindow, QTableWidget, QTableWidgetItem, QAction, QFileDialog, QMessageBox

file_path =".env"

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        # Load .env file
        env_vars = {}
        with open(file_path , 'r') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    name, value = line.split('=', 1)
                    env_vars[name] = value

        # Create table widget
        self.table = QTableWidget()
        self.setCentralWidget(self.table)

        # Set column headers
        self.table.setColumnCount(2)
        self.table.setHorizontalHeaderLabels(['Name', 'Value'])


		# Set row height and column width
        self.table.verticalHeader().setDefaultSectionSize(24)
        self.table.horizontalHeader().setDefaultSectionSize(150)
        
        # Populate table with .env name and value pairs
        for i, (name, value) in enumerate(env_vars.items()):
            self.table.insertRow(i)
            self.table.setItem(i, 0, QTableWidgetItem(name))
            self.table.setItem(i, 1, QTableWidgetItem(value))

        # Add "Save" action to menu bar
        save_action = QAction('Save', self)
        save_action.triggered.connect(self.save_env_file)
        self.menuBar().addAction(save_action)

    def save_env_file(self):
        # Get file path from user
        #file_path, _ = QFileDialog.getSaveFileName(self, 'Save .env File', filter='*.env')

        if file_path:
            # Save .env file
            with open(file_path, 'w') as f:
                for i in range(self.table.rowCount()):
                    name_item = self.table.item(i, 0)
                    value_item = self.table.item(i, 1)
                    if name_item and value_item:
                        name = name_item.text()
                        value = value_item.text()
                        f.write(f'{name}={value}\n')
            
			# Show success message
            message_box = QMessageBox()
            message_box.setWindowTitle('Success')
            message_box.setText('The .env file was saved successfully.')
            message_box.setIcon(QMessageBox.Information)
            message_box.exec_()


if __name__ == '__main__':
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())