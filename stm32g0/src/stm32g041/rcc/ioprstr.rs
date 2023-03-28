#[doc = "Register `IOPRSTR` reader"]
pub struct R(crate::R<IOPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPRSTR` writer"]
pub struct W(crate::W<IOPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPRSTR_SPEC>;
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
impl From<crate::W<IOPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = crate::BitReader<bool>;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IOPARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub type IOPBRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub type IOPBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub type IOPCRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub type IOPCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub type IOPDRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub type IOPDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub type IOPFRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub type IOPFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<0> {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<1> {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<2> {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<3> {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<5> {
        IOPFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioprstr](index.html) module"]
pub struct IOPRSTR_SPEC;
impl crate::RegisterSpec for IOPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioprstr::R](R) reader structure"]
impl crate::Readable for IOPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioprstr::W](W) writer structure"]
impl crate::Writable for IOPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IOPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
