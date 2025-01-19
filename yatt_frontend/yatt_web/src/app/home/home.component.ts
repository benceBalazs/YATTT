import {AfterViewInit, Component, OnInit, ViewChild} from '@angular/core';
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
import {MatSort, MatSortHeader, Sort} from '@angular/material/sort';
import {MatPaginator} from '@angular/material/paginator';
import {ThemeService} from '../theme.service';

@Component({
  selector: 'app-home',
  imports: [
    MatIcon,
    MatTable,
    MatHeaderCell,
    MatHeaderRow,
    MatRow,
    MatCell,
    MatColumnDef,
    MatSort,
    MatSortHeader,
    MatPaginator,
  ],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent implements AfterViewInit {
  isDarkMode: boolean;

  constructor(private themeService: ThemeService) {
    this.isDarkMode = this.themeService.isDarkMode();
  }

  toggleTheme() {
    this.isDarkMode = !this.isDarkMode;
    this.themeService.setDarkMode(this.isDarkMode);
  }

  displayedColumns: string[] = ['checkIn', 'checkOut', 'duration', 'card', 'course'];

  // Use MatTableDataSource to enable sorting and pagination
  dataSource = new MatTableDataSource([
    { checkIn: '08:00', checkOut: '10:00', duration: '2h 0m', card: '489235', course: 'Advanced Software Engineering' },
    { checkIn: '10:15', checkOut: '12:15', duration: '2h 0m', card: '1234123', course: 'Security Engineering' },
    { checkIn: '13:00', checkOut: '15:30', duration: '2h 30m', card: '42356425', course: 'Technology Assessment' },
    { checkIn: '15:45', checkOut: '17:15', duration: '1h 30m', card: '1234123', course: 'Distributed Computing Infrastructures' },
    { checkIn: '18:00', checkOut: '20:00', duration: '2h 0m', card: '505132412', course: 'Introduction to Artificial Intelligence' },
    { checkIn: '08:15', checkOut: '09:45', duration: '1h 30m', card: '829348509', course: 'Interoperability' },
    { checkIn: '08:00', checkOut: '10:00', duration: '2h 0m', card: '829348509', course: 'Seminar in Business Information Systems' },
    { checkIn: '13:00', checkOut: '15:30', duration: '2h 30m', card: '1234123', course: 'Current Topics in Multimedia Systems: Adaptive Media Streaming' },
    { checkIn: '15:45', checkOut: '17:15', duration: '1h 30m', card: '1234123', course: 'Current Topics in Multimedia Systems' },
    { checkIn: '08:15', checkOut: '09:45', duration: '1h 30m', card: '489235', course: 'Scientific Writing' },
    {checkIn: '09:45', checkOut: '12:00', duration: '2h 15m', card: '2348394', course: 'Human Computer Interaction'},
    {checkIn: '11:45', checkOut: '13:15', duration: '1h 30m', card:'35738943', course: 'Software Testing'},
  ]);

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  ngAfterViewInit() {
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
  }


  sortCourse(sort:Sort){

  }
  }

