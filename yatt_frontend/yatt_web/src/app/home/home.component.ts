import {Component} from '@angular/core';
import {MatIcon} from '@angular/material/icon';
import {
  MatCell,
  MatCellDef,
  MatColumnDef,
  MatHeaderCell, MatHeaderCellDef,
  MatHeaderRow, MatHeaderRowDef,
  MatRow, MatRowDef,
  MatTable, MatTableDataSource
} from '@angular/material/table';
import {MatSort} from '@angular/material/sort';

@Component({
  selector: 'app-home',
  imports: [
    MatIcon,
    MatTable,
    MatHeaderCell,
    MatHeaderRow,
    MatRow,
    MatCellDef,
    MatCell,
    MatColumnDef,
    MatHeaderCellDef,
    MatHeaderRowDef,
    MatRowDef,
    MatSort,
  ],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {
  displayedColumns: string[] = ['checkIn', 'checkOut', 'duration', 'card', 'course'];

  dataSource = [
    { checkIn: '08:00 AM', checkOut: '10:00 AM', duration: '2h 0m', card: '489235', course: 'Advanced Software Engineering' },
    { checkIn: '10:15 AM', checkOut: '12:15 PM', duration: '2h 0m', card: '1234123', course: 'Security Engineering' },
    { checkIn: '01:00 PM', checkOut: '03:30 PM', duration: '2h 30m', card: '42356425', course: 'Technology Assesment' },
    { checkIn: '03:45 PM', checkOut: '05:15 PM', duration: '1h 30m', card: '1234123', course: 'Distributed Computing Infrastructures' },
    { checkIn: '06:00 PM', checkOut: '08:00 PM', duration: '2h 0m', card: '505132412', course: 'Introduction to Artificial Intelligence' }
  ]
}

