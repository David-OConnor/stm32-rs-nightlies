#[doc = "Register `CCR3` reader"]
pub struct R(crate::R<CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR3` writer"]
pub struct W(crate::W<CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR3_SPEC>;
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
impl From<crate::W<CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - channel enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - channel enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `HTIE` reader - HTIE"]
pub type HTIE_R = crate::BitReader<bool>;
#[doc = "Field `HTIE` writer - HTIE"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `CIRC` reader - CIRC"]
pub type CIRC_R = crate::BitReader<bool>;
#[doc = "Field `CIRC` writer - CIRC"]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `PINC` reader - PINC"]
pub type PINC_R = crate::BitReader<bool>;
#[doc = "Field `PINC` writer - PINC"]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `MINC` reader - MINC"]
pub type MINC_R = crate::BitReader<bool>;
#[doc = "Field `MINC` writer - MINC"]
pub type MINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
#[doc = "Field `PSIZE` reader - PSIZE"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - PSIZE"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `MSIZE` reader - MSIZE"]
pub type MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIZE` writer - MSIZE"]
pub type MSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PL` reader - PL"]
pub type PL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PL` writer - PL"]
pub type PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `MEM2MEM` reader - MEM2MEM"]
pub type MEM2MEM_R = crate::BitReader<bool>;
#[doc = "Field `MEM2MEM` writer - MEM2MEM"]
pub type MEM2MEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIE"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - HTIE"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<2> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 3 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<3> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<5> {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<6> {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<7> {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<10> {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - PL"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<12> {
        PL_W::new(self)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<14> {
        MEM2MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](index.html) module"]
pub struct CCR3_SPEC;
impl crate::RegisterSpec for CCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr3::R](R) reader structure"]
impl crate::Readable for CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr3::W](W) writer structure"]
impl crate::Writable for CCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
