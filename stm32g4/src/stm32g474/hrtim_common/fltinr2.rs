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
#[doc = "Field `FLT6E` reader - FLT6E"]
pub type FLT6E_R = crate::BitReader<bool>;
#[doc = "Field `FLT6E` writer - FLT6E"]
pub type FLT6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT6P` reader - FLT6P"]
pub type FLT6P_R = crate::BitReader<bool>;
#[doc = "Field `FLT6P` writer - FLT6P"]
pub type FLT6P_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT6SRC_0` reader - FLT6F"]
pub type FLT6SRC_0_R = crate::BitReader<bool>;
#[doc = "Field `FLT6SRC_0` writer - FLT6F"]
pub type FLT6SRC_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT6F` reader - FLT6F"]
pub type FLT6F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT6F` writer - FLT6F"]
pub type FLT6F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTINR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLT6LCK` reader - FLT6LCK"]
pub type FLT6LCK_R = crate::BitReader<bool>;
#[doc = "Field `FLT6LCK` writer - FLT6LCK"]
pub type FLT6LCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT1SRC_1` reader - FLT1SRC_1"]
pub type FLT1SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT1SRC_1` writer - FLT1SRC_1"]
pub type FLT1SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT2SRC_1` reader - FLT2SRC_1"]
pub type FLT2SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT2SRC_1` writer - FLT2SRC_1"]
pub type FLT2SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT3SRC_1` reader - FLT3SRC_1"]
pub type FLT3SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT3SRC_1` writer - FLT3SRC_1"]
pub type FLT3SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT4SRC_1` reader - FLT4SRC_1"]
pub type FLT4SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT4SRC_1` writer - FLT4SRC_1"]
pub type FLT4SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT5SRC_1` reader - FLT5SRC_1"]
pub type FLT5SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT5SRC_1` writer - FLT5SRC_1"]
pub type FLT5SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
#[doc = "Field `FLT6SRC_1` reader - FLT6SRC"]
pub type FLT6SRC_1_R = crate::BitReader<bool>;
#[doc = "Field `FLT6SRC_1` writer - FLT6SRC"]
pub type FLT6SRC_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTINR2_SPEC, bool, O>;
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
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&self) -> FLT6E_R {
        FLT6E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&self) -> FLT6P_R {
        FLT6P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&self) -> FLT6SRC_0_R {
        FLT6SRC_0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&self) -> FLT6F_R {
        FLT6F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&self) -> FLT6LCK_R {
        FLT6LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&self) -> FLT1SRC_1_R {
        FLT1SRC_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&self) -> FLT2SRC_1_R {
        FLT2SRC_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&self) -> FLT3SRC_1_R {
        FLT3SRC_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&self) -> FLT4SRC_1_R {
        FLT4SRC_1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&self) -> FLT5SRC_1_R {
        FLT5SRC_1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&self) -> FLT6SRC_1_R {
        FLT6SRC_1_R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    #[must_use]
    pub fn flt6e(&mut self) -> FLT6E_W<8> {
        FLT6E_W::new(self)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    #[must_use]
    pub fn flt6p(&mut self) -> FLT6P_W<9> {
        FLT6P_W::new(self)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_0(&mut self) -> FLT6SRC_0_W<10> {
        FLT6SRC_0_W::new(self)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6f(&mut self) -> FLT6F_W<11> {
        FLT6F_W::new(self)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt6lck(&mut self) -> FLT6LCK_W<15> {
        FLT6LCK_W::new(self)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src_1(&mut self) -> FLT1SRC_1_W<16> {
        FLT1SRC_1_W::new(self)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src_1(&mut self) -> FLT2SRC_1_W<17> {
        FLT2SRC_1_W::new(self)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src_1(&mut self) -> FLT3SRC_1_W<18> {
        FLT3SRC_1_W::new(self)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src_1(&mut self) -> FLT4SRC_1_W<19> {
        FLT4SRC_1_W::new(self)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src_1(&mut self) -> FLT5SRC_1_W<20> {
        FLT5SRC_1_W::new(self)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_1(&mut self) -> FLT6SRC_1_W<21> {
        FLT6SRC_1_W::new(self)
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
