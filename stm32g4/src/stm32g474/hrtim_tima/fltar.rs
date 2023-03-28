#[doc = "Register `FLTAR` reader"]
pub struct R(crate::R<FLTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTAR` writer"]
pub struct W(crate::W<FLTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTAR_SPEC>;
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
impl From<crate::W<FLTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type FLT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type FLT3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type FLT4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type FLT5EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type FLT5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLT6EN` reader - Fault 6 enable"]
pub type FLT6EN_R = crate::BitReader<bool>;
#[doc = "Field `FLT6EN` writer - Fault 6 enable"]
pub type FLT6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FLTLCK_R = crate::BitReader<bool>;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FLTLCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    pub fn flt6en(&self) -> FLT6EN_R {
        FLT6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<0> {
        FLT1EN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<1> {
        FLT2EN_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<2> {
        FLT3EN_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<3> {
        FLT4EN_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<4> {
        FLT5EN_W::new(self)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt6en(&mut self) -> FLT6EN_W<5> {
        FLT6EN_W::new(self)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<31> {
        FLTLCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltar](index.html) module"]
pub struct FLTAR_SPEC;
impl crate::RegisterSpec for FLTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltar::R](R) reader structure"]
impl crate::Readable for FLTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltar::W](W) writer structure"]
impl crate::Writable for FLTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTAR to value 0"]
impl crate::Resettable for FLTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
