#[doc = "Register `RGCFR` reader"]
pub struct R(crate::R<RGCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGCFR` writer"]
pub struct W(crate::W<RGCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCFR_SPEC>;
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
impl From<crate::W<RGCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` reader - Generator Clear Overrun Flag 0"]
pub type CSOF0_R = crate::BitReader<bool>;
#[doc = "Field `CSOF0` writer - Generator Clear Overrun Flag 0"]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `CSOF1` reader - Generator Clear Overrun Flag 1"]
pub type CSOF1_R = crate::BitReader<bool>;
#[doc = "Field `CSOF1` writer - Generator Clear Overrun Flag 1"]
pub type CSOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `CSOF2` reader - Generator Clear Overrun Flag 2"]
pub type CSOF2_R = crate::BitReader<bool>;
#[doc = "Field `CSOF2` writer - Generator Clear Overrun Flag 2"]
pub type CSOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
#[doc = "Field `CSOF3` reader - Generator Clear Overrun Flag 3"]
pub type CSOF3_R = crate::BitReader<bool>;
#[doc = "Field `CSOF3` writer - Generator Clear Overrun Flag 3"]
pub type CSOF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Generator Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generator Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generator Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generator Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generator Clear Overrun Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<0> {
        CSOF0_W::new(self)
    }
    #[doc = "Bit 1 - Generator Clear Overrun Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<1> {
        CSOF1_W::new(self)
    }
    #[doc = "Bit 2 - Generator Clear Overrun Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<2> {
        CSOF2_W::new(self)
    }
    #[doc = "Bit 3 - Generator Clear Overrun Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<3> {
        CSOF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Request Generator Clear Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](index.html) module"]
pub struct RGCFR_SPEC;
impl crate::RegisterSpec for RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgcfr::R](R) reader structure"]
impl crate::Readable for RGCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgcfr::W](W) writer structure"]
impl crate::Writable for RGCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
