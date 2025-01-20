import {AfterViewInit, Component, OnInit, ViewChild} from '@angular/core';
import {MatIcon, MatIconModule} from '@angular/material/icon';
import {
  MatCell,
  MatCellDef,
  MatColumnDef,
  MatHeaderCell, MatHeaderCellDef,
  MatHeaderRow, MatHeaderRowDef,
  MatRow, MatRowDef,
  MatTable, MatTableDataSource, MatTableModule
} from '@angular/material/table';
import {MatSort, MatSortHeader, MatSortModule, Sort} from '@angular/material/sort';
import {MatPaginator, MatPaginatorModule} from '@angular/material/paginator';
import {DataApiService} from '../data-api.service';
import {Router} from '@angular/router';
import {HttpClient} from '@angular/common/http';

@Component({
  selector: 'app-home',
  imports: [
  MatTableModule,
  MatSortModule,
  MatPaginatorModule,
  MatIconModule
],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent implements AfterViewInit {

  constructor(private dataApiService: DataApiService, private router: Router, private http: HttpClient) {
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

