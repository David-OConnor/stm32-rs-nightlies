#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Bank 1 configuration lock bit"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Bank 1 configuration lock bit"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PG` reader - Bank 1 internal buffer control bit"]
pub type PG_R = crate::BitReader<bool>;
#[doc = "Field `PG` writer - Bank 1 internal buffer control bit"]
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SER` reader - Bank 1 sector erase request"]
pub type SER_R = crate::BitReader<bool>;
#[doc = "Field `SER` writer - Bank 1 sector erase request"]
pub type SER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BER` reader - Bank 1 erase request"]
pub type BER_R = crate::BitReader<bool>;
#[doc = "Field `BER` writer - Bank 1 erase request"]
pub type BER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PSIZE` reader - Bank 1 program size"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - Bank 1 program size"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FW` reader - Bank 1 write forcing control bit"]
pub type FW_R = crate::BitReader<bool>;
#[doc = "Field `FW` writer - Bank 1 write forcing control bit"]
pub type FW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START` reader - Bank 1 erase start control bit"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Bank 1 erase start control bit"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SNB` reader - Bank 1 sector erase selection number"]
pub type SNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNB` writer - Bank 1 sector erase selection number"]
pub type SNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CRC_EN` reader - Bank 1 CRC control bit"]
pub type CRC_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRC_EN` writer - Bank 1 CRC control bit"]
pub type CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EOPIE` reader - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOPIE` writer - Bank 1 end-of-program interrupt control bit"]
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WRPERRIE` reader - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE_R = crate::BitReader<bool>;
#[doc = "Field `WRPERRIE` writer - Bank 1 write protection error interrupt enable bit"]
pub type WRPERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PGSERRIE` reader - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PGSERRIE` writer - Bank 1 programming sequence error interrupt enable bit"]
pub type PGSERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STRBERRIE` reader - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE_R = crate::BitReader<bool>;
#[doc = "Field `STRBERRIE` writer - Bank 1 strobe error interrupt enable bit"]
pub type STRBERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `INCERRIE` reader - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `INCERRIE` writer - Bank 1 inconsistency error interrupt enable bit"]
pub type INCERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OPERRIE` reader - Bank 1 write"]
pub type OPERRIE_R = crate::BitReader<bool>;
#[doc = "Field `OPERRIE` writer - Bank 1 write"]
pub type OPERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RDPERRIE` reader - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RDPERRIE` writer - Bank 1 read protection error interrupt enable bit"]
pub type RDPERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RDSERRIE` reader - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RDSERRIE` writer - Bank 1 secure error interrupt enable bit"]
pub type RDSERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SNECCERRIE` reader - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `SNECCERRIE` writer - Bank 1 ECC single correction error interrupt enable bit"]
pub type SNECCERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBECCERRIE` reader - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE_R = crate::BitReader<bool>;
#[doc = "Field `DBECCERRIE` writer - Bank 1 ECC double detection error interrupt enable bit"]
pub type DBECCERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CRCENDIE` reader - Bank 1 CRC end of calculation interrupt enable bit"]
pub type CRCENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCENDIE` writer - Bank 1 CRC end of calculation interrupt enable bit"]
pub type CRCENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CRCRDERRIE` reader - Bank 1 CRC read error interrupt enable bit"]
pub type CRCRDERRIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCRDERRIE` writer - Bank 1 CRC read error interrupt enable bit"]
pub type CRCRDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 internal buffer control bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw(&self) -> FW_R {
        FW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 erase start control bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CRC_EN_R {
        CRC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie(&self) -> RDPERRIE_R {
        RDPERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie(&self) -> RDSERRIE_R {
        RDSERRIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie(&self) -> SNECCERRIE_R {
        SNECCERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie(&self) -> DBECCERRIE_R {
        DBECCERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie(&self) -> CRCENDIE_R {
        CRCENDIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error interrupt enable bit"]
    #[inline(always)]
    pub fn crcrderrie(&self) -> CRCRDERRIE_R {
        CRCRDERRIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - Bank 1 internal buffer control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<1> {
        PG_W::new(self)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<2> {
        SER_W::new(self)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    #[must_use]
    pub fn ber(&mut self) -> BER_W<3> {
        BER_W::new(self)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<4> {
        PSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw(&mut self) -> FW_W<6> {
        FW_W::new(self)
    }
    #[doc = "Bit 7 - Bank 1 erase start control bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<7> {
        START_W::new(self)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    #[must_use]
    pub fn snb(&mut self) -> SNB_W<8> {
        SNB_W::new(self)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc_en(&mut self) -> CRC_EN_W<15> {
        CRC_EN_W::new(self)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<16> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wrperrie(&mut self) -> WRPERRIE_W<17> {
        WRPERRIE_W::new(self)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgserrie(&mut self) -> PGSERRIE_W<18> {
        PGSERRIE_W::new(self)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn strberrie(&mut self) -> STRBERRIE_W<19> {
        STRBERRIE_W::new(self)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn incerrie(&mut self) -> INCERRIE_W<21> {
        INCERRIE_W::new(self)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    #[must_use]
    pub fn operrie(&mut self) -> OPERRIE_W<22> {
        OPERRIE_W::new(self)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdperrie(&mut self) -> RDPERRIE_W<23> {
        RDPERRIE_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdserrie(&mut self) -> RDSERRIE_W<24> {
        RDSERRIE_W::new(self)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sneccerrie(&mut self) -> SNECCERRIE_W<25> {
        SNECCERRIE_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbeccerrie(&mut self) -> DBECCERRIE_W<26> {
        DBECCERRIE_W::new(self)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcendie(&mut self) -> CRCENDIE_W<27> {
        CRCENDIE_W::new(self)
    }
    #[doc = "Bit 28 - Bank 1 CRC read error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcrderrie(&mut self) -> CRCRDERRIE_W<28> {
        CRCRDERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x31"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x31;
}
