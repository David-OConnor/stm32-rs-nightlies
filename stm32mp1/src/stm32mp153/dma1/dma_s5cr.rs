#[doc = "Register `DMA_S5CR` reader"]
pub struct R(crate::R<DMA_S5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S5CR` writer"]
pub struct W(crate::W<DMA_S5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S5CR_SPEC>;
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
impl From<crate::W<DMA_S5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S5CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `DMEIE` reader - DMEIE"]
pub type DMEIE_R = crate::BitReader<bool>;
#[doc = "Field `DMEIE` writer - DMEIE"]
pub type DMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `HTIE` reader - HTIE"]
pub type HTIE_R = crate::BitReader<bool>;
#[doc = "Field `HTIE` writer - HTIE"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `PFCTRL` reader - PFCTRL"]
pub type PFCTRL_R = crate::BitReader<bool>;
#[doc = "Field `PFCTRL` writer - PFCTRL"]
pub type PFCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CIRC` reader - CIRC"]
pub type CIRC_R = crate::BitReader<bool>;
#[doc = "Field `CIRC` writer - CIRC"]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `PINC` reader - PINC"]
pub type PINC_R = crate::BitReader<bool>;
#[doc = "Field `PINC` writer - PINC"]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `MINC` reader - MINC"]
pub type MINC_R = crate::BitReader<bool>;
#[doc = "Field `MINC` writer - MINC"]
pub type MINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `PSIZE` reader - PSIZE"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - PSIZE"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MSIZE` reader - MSIZE"]
pub type MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIZE` writer - MSIZE"]
pub type MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PINCOS` reader - PINCOS"]
pub type PINCOS_R = crate::BitReader<bool>;
#[doc = "Field `PINCOS` writer - PINCOS"]
pub type PINCOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `PL` reader - PL"]
pub type PL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PL` writer - PL"]
pub type PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DBM` reader - DBM"]
pub type DBM_R = crate::BitReader<bool>;
#[doc = "Field `DBM` writer - DBM"]
pub type DBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `CT` reader - CT"]
pub type CT_R = crate::BitReader<bool>;
#[doc = "Field `CT` writer - CT"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_S5CR_SPEC, bool, O>;
#[doc = "Field `PBURST` reader - PBURST"]
pub type PBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBURST` writer - PBURST"]
pub type PBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MBURST` reader - MBURST"]
pub type MBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBURST` writer - MBURST"]
pub type MBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S5CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn dmeie(&mut self) -> DMEIE_W<1> {
        DMEIE_W::new(self)
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<3> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<4> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<5> {
        PFCTRL_W::new(self)
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<6> {
        DIR_W::new(self)
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<8> {
        CIRC_W::new(self)
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<9> {
        PINC_W::new(self)
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<10> {
        MINC_W::new(self)
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<11> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<13> {
        MSIZE_W::new(self)
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<15> {
        PINCOS_W::new(self)
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<16> {
        PL_W::new(self)
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<18> {
        DBM_W::new(self)
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<19> {
        CT_W::new(self)
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<21> {
        PBURST_W::new(self)
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<23> {
        MBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the concerned stream.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s5cr](index.html) module"]
pub struct DMA_S5CR_SPEC;
impl crate::RegisterSpec for DMA_S5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s5cr::R](R) reader structure"]
impl crate::Readable for DMA_S5CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s5cr::W](W) writer structure"]
impl crate::Writable for DMA_S5CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_S5CR to value 0"]
impl crate::Resettable for DMA_S5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
