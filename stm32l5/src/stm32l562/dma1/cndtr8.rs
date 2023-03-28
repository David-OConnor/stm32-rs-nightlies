#[doc = "Register `CNDTR8` reader"]
pub struct R(crate::R<CNDTR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR8` writer"]
pub struct W(crate::W<CNDTR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR8_SPEC>;
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
impl From<crate::W<CNDTR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - channel enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - channel enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR8_SPEC, u16, u16, 16, O>;
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `HTIE` reader - half transfer interrupt enable"]
pub type HTIE_R = crate::BitReader<bool>;
#[doc = "Field `HTIE` writer - half transfer interrupt enable"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `TEIE` reader - transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - transfer error interrupt enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `DIR` reader - data transfer direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - data transfer direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `CIRC` reader - circular mode"]
pub type CIRC_R = crate::BitReader<bool>;
#[doc = "Field `CIRC` writer - circular mode"]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `PINC` reader - peripheral increment mode"]
pub type PINC_R = crate::BitReader<bool>;
#[doc = "Field `PINC` writer - peripheral increment mode"]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `MINC` reader - memory increment mode"]
pub type MINC_R = crate::BitReader<bool>;
#[doc = "Field `MINC` writer - memory increment mode"]
pub type MINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `PSIZE` reader - peripheral size"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - peripheral size"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR8_SPEC, u8, u8, 2, O>;
#[doc = "Field `MSIZE` reader - memory size"]
pub type MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIZE` writer - memory size"]
pub type MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR8_SPEC, u8, u8, 2, O>;
#[doc = "Field `PL` reader - priority level"]
pub type PL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PL` writer - priority level"]
pub type PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR8_SPEC, u8, u8, 2, O>;
#[doc = "Field `MEM2MEM` reader - memory-to-memory mode"]
pub type MEM2MEM_R = crate::BitReader<bool>;
#[doc = "Field `MEM2MEM` writer - memory-to-memory mode"]
pub type MEM2MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `DBM` reader - double-buffer mode"]
pub type DBM_R = crate::BitReader<bool>;
#[doc = "Field `DBM` writer - double-buffer mode"]
pub type DBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `CT` reader - current target memory of DMA transfer in double-buffer mode"]
pub type CT_R = crate::BitReader<bool>;
#[doc = "Field `CT` writer - current target memory of DMA transfer in double-buffer mode"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `SECM` reader - secure mode"]
pub type SECM_R = crate::BitReader<bool>;
#[doc = "Field `SECM` writer - secure mode"]
pub type SECM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `SSEC` reader - security of the DMA transfer from the source"]
pub type SSEC_R = crate::BitReader<bool>;
#[doc = "Field `SSEC` writer - security of the DMA transfer from the source"]
pub type SSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `DSEC` reader - security of the DMA transfer to the destination"]
pub type DSEC_R = crate::BitReader<bool>;
#[doc = "Field `DSEC` writer - security of the DMA transfer to the destination"]
pub type DSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
#[doc = "Field `PRIV` reader - privileged mode"]
pub type PRIV_R = crate::BitReader<bool>;
#[doc = "Field `PRIV` writer - privileged mode"]
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDTR8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - memory-to-memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - double-buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - current target memory of DMA transfer in double-buffer mode"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - secure mode"]
    #[inline(always)]
    pub fn secm(&self) -> SECM_R {
        SECM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - security of the DMA transfer from the source"]
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - security of the DMA transfer to the destination"]
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - privileged mode"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<2> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 3 - transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<3> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 4 - data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<5> {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<6> {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<7> {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - peripheral size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - memory size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<10> {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<12> {
        PL_W::new(self)
    }
    #[doc = "Bit 14 - memory-to-memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<14> {
        MEM2MEM_W::new(self)
    }
    #[doc = "Bit 15 - double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<15> {
        DBM_W::new(self)
    }
    #[doc = "Bit 16 - current target memory of DMA transfer in double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<16> {
        CT_W::new(self)
    }
    #[doc = "Bit 17 - secure mode"]
    #[inline(always)]
    #[must_use]
    pub fn secm(&mut self) -> SECM_W<17> {
        SECM_W::new(self)
    }
    #[doc = "Bit 18 - security of the DMA transfer from the source"]
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<18> {
        SSEC_W::new(self)
    }
    #[doc = "Bit 19 - security of the DMA transfer to the destination"]
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<19> {
        DSEC_W::new(self)
    }
    #[doc = "Bit 20 - privileged mode"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<20> {
        PRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr8](index.html) module"]
pub struct CNDTR8_SPEC;
impl crate::RegisterSpec for CNDTR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr8::R](R) reader structure"]
impl crate::Readable for CNDTR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr8::W](W) writer structure"]
impl crate::Writable for CNDTR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDTR8 to value 0"]
impl crate::Resettable for CNDTR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
