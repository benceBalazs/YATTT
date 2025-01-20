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
import {DataApiService} from '../data-api.service';
import {Router} from '@angular/router';
import {HttpClient} from '@angular/common/http';

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

  constructor(private themeService: ThemeService, private dataApiService: DataApiService, private router: Router, private http: HttpClient) {
    this.isDarkMode = this.themeService.isDarkMode();
  }

  toggleTheme() {
    this.isDarkMode = !this.isDarkMode;
    this.themeService.setDarkMode(this.isDarkMode);
  }

  displayedColumns: string[] = ['check_in_time', 'check_out_time', 'duration', 'card_name', 'lecture_name'];

  // Use MatTableDataSource to enable sorting and pagination
  dataSource = new MatTableDataSource([]);

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  ngOnInit() {
    this.loadData();
  }
  ngAfterViewInit() {
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
  }


  sortCourse(sort:Sort){

  }

  private loadData() {
    this.dataApiService.getAttendances().subscribe(data => {
      this.dataSource.data = data; // Assign the fetched data to MatTableDataSource
    }, error => {
      console.error('Error fetching attendance data', error); // Handle any errors
    });
  }

}

