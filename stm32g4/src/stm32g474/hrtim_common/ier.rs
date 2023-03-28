#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT1IE` reader - Fault 1 Interrupt Enable"]
pub type FLT1IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT1IE` writer - Fault 1 Interrupt Enable"]
pub type FLT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `FLT2IE` reader - Fault 2 Interrupt Enable"]
pub type FLT2IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT2IE` writer - Fault 2 Interrupt Enable"]
pub type FLT2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `FLT3IE` reader - Fault 3 Interrupt Enable"]
pub type FLT3IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT3IE` writer - Fault 3 Interrupt Enable"]
pub type FLT3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `FLT4IE` reader - Fault 4 Interrupt Enable"]
pub type FLT4IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT4IE` writer - Fault 4 Interrupt Enable"]
pub type FLT4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `FLT5IE` reader - Fault 5 Interrupt Enable"]
pub type FLT5IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT5IE` writer - Fault 5 Interrupt Enable"]
pub type FLT5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SYSFLTE` reader - System Fault Interrupt Enable"]
pub type SYSFLTE_R = crate::BitReader<bool>;
#[doc = "Field `SYSFLTE` writer - System Fault Interrupt Enable"]
pub type SYSFLTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `FLT6IE` reader - Fault 6 Interrupt Enable"]
pub type FLT6IE_R = crate::BitReader<bool>;
#[doc = "Field `FLT6IE` writer - Fault 6 Interrupt Enable"]
pub type FLT6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `DLLRDYIE` reader - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `DLLRDYIE` writer - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BMPERIE` reader - Burst mode period Interrupt Enable"]
pub type BMPERIE_R = crate::BitReader<bool>;
#[doc = "Field `BMPERIE` writer - Burst mode period Interrupt Enable"]
pub type BMPERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysflte(&self) -> SYSFLTE_R {
        SYSFLTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Enable"]
    #[inline(always)]
    pub fn flt6ie(&self) -> FLT6IE_R {
        FLT6IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&self) -> DLLRDYIE_R {
        DLLRDYIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1ie(&mut self) -> FLT1IE_W<0> {
        FLT1IE_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2ie(&mut self) -> FLT2IE_W<1> {
        FLT2IE_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3ie(&mut self) -> FLT3IE_W<2> {
        FLT3IE_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4ie(&mut self) -> FLT4IE_W<3> {
        FLT4IE_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5ie(&mut self) -> FLT5IE_W<4> {
        FLT5IE_W::new(self)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysflte(&mut self) -> SYSFLTE_W<5> {
        SYSFLTE_W::new(self)
    }
    #[doc = "Bit 6 - Fault 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt6ie(&mut self) -> FLT6IE_W<6> {
        FLT6IE_W::new(self)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyie(&mut self) -> DLLRDYIE_W<16> {
        DLLRDYIE_W::new(self)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BMPERIE_W<17> {
        BMPERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
