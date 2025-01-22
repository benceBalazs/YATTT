import {Component, OnInit, ViewChild} from '@angular/core';
import { DataApiService } from '../../services/data-api.service';
import { Card, CardWithId } from '../project.model';
import {FormsModule} from '@angular/forms';
import {NgForOf, NgIf} from '@angular/common';
import {MatPaginator} from '@angular/material/paginator';
import {MatSort, Sort} from '@angular/material/sort';
import {
  MatCell,
  MatCellDef,
  MatColumnDef,
  MatHeaderCell,
  MatHeaderCellDef,
  MatHeaderRow, MatHeaderRowDef, MatRow, MatRowDef,
  MatTable, MatTableDataSource
} from '@angular/material/table';

@Component({
  selector: 'app-cards',
  templateUrl: './cards.component.html',
  styleUrls: ['./cards.component.css'],
  imports: [
    FormsModule,
    NgIf,
    NgForOf,
    MatPaginator,
    MatSort,
    MatTable,
    MatCell,
    MatHeaderCell,
    MatColumnDef,
    MatHeaderCellDef,
    MatCellDef,
    MatHeaderRow,
    MatRow,
    MatHeaderRowDef,
    MatRowDef
  ]
})
export class CardsComponent implements OnInit {
  cards: CardWithId[] = [];
  cardForm: CardWithId = { card_name: '', tag_id: '' };
  showModal = false;
  isEdit = false;
  displayedColumns: string[] = ['card_name', 'tag_id', 'actions'];
  dataSource = new MatTableDataSource<CardWithId>([]);

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  constructor(private dataApiService: DataApiService) {}

  ngOnInit(): void {
    this.fetchCards();
  }

  ngAfterViewInit() {
    console.log('ngAfterViewInit called');
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
    console.log('Paginator:', this.paginator);
    console.log('Sort:', this.sort);

  }

  sortCourse(sort: Sort) {}

  fetchCards(): void {
    this.dataApiService.getCards().subscribe(
      (cards) => {
        this.cards = cards.cards; // Assuming response contains a `cards` property
        this.dataSource.data = this.cards;
        this.dataSource.paginator = this.paginator; // Reassign after data load
        this.dataSource.sort = this.sort;
      },
      (error) => {
        console.error('Error fetching cards:', error);
      }
    );
  }

  openAddModal(): void {
    this.isEdit = false;
    this.cardForm = { card_name: '', tag_id: this.generateTagId() };
    this.showModal = true;
  }

  openEditModal(card: CardWithId): void {
    this.isEdit = true;
    this.cardForm = { ...card };
    this.showModal = true;
  }

  closeModal(): void {
    this.showModal = false;
  }

  saveCard(): void {
    if (this.isEdit) {
      this.dataApiService.updateCard(this.cardForm.id!.id.String, this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    } else {
      this.dataApiService.addCard(this.cardForm).subscribe(() => {
        this.fetchCards();
        this.closeModal();
      });
    }
  }

  deleteCard(card: CardWithId): void {
    this.dataApiService.deleteCard(card.id!.id.String).subscribe(() => {
      this.fetchCards();
    });
  }

  generateTagId(): string {
    return "0";
  }
}
