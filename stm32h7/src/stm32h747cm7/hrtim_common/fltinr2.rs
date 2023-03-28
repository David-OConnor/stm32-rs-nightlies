#[doc = "Register `FLTINR2` reader"]
pub struct R(crate::R<FLTINR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR2` writer"]
pub struct W(crate::W<FLTINR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR2_SPEC>;
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
impl From<crate::W<FLTINR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT5E` reader - FLT5E"]
pub type FLT5E_R = crate::BitReader<bool>;
#[doc = "Field `FLT5E` writer - FLT5E"]
pub type FLT5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT5P` reader - FLT5P"]
pub type FLT5P_R = crate::BitReader<bool>;
#[doc = "Field `FLT5P` writer - FLT5P"]
pub type FLT5P_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub type FLT5SRC_R = crate::BitReader<bool>;
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub type FLT5SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT5F` reader - FLT5F"]
pub type FLT5F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT5F` writer - FLT5F"]
pub type FLT5F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub type FLT5LCK_R = crate::BitReader<bool>;
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub type FLT5LCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLTSD` reader - FLTSD"]
pub type FLTSD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTSD` writer - FLTSD"]
pub type FLTSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> FLT5E_W<0> {
        FLT5E_W::new(self)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> FLT5P_W<1> {
        FLT5P_W::new(self)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> FLT5SRC_W<2> {
        FLT5SRC_W::new(self)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> FLT5F_W<3> {
        FLT5F_W::new(self)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<7> {
        FLT5LCK_W::new(self)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FLTSD_W<24> {
        FLTSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr2](index.html) module"]
pub struct FLTINR2_SPEC;
impl crate::RegisterSpec for FLTINR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr2::R](R) reader structure"]
impl crate::Readable for FLTINR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr2::W](W) writer structure"]
impl crate::Writable for FLTINR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for FLTINR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
