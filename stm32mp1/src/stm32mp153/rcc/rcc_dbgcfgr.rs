#[doc = "Register `RCC_DBGCFGR` reader"]
pub struct R(crate::R<RCC_DBGCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_DBGCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_DBGCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_DBGCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_DBGCFGR` writer"]
pub struct W(crate::W<RCC_DBGCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_DBGCFGR_SPEC>;
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
impl From<crate::W<RCC_DBGCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_DBGCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEDIV` reader - TRACEDIV"]
pub type TRACEDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRACEDIV` writer - TRACEDIV"]
pub type TRACEDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_DBGCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBGCKEN` reader - DBGCKEN"]
pub type DBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGCKEN` writer - DBGCKEN"]
pub type DBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DBGCFGR_SPEC, bool, O>;
#[doc = "Field `TRACECKEN` reader - TRACECKEN"]
pub type TRACECKEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACECKEN` writer - TRACECKEN"]
pub type TRACECKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DBGCFGR_SPEC, bool, O>;
#[doc = "Field `DBGRST` reader - DBGRST"]
pub type DBGRST_R = crate::BitReader<bool>;
#[doc = "Field `DBGRST` writer - DBGRST"]
pub type DBGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_DBGCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    #[must_use]
    pub fn tracediv(&mut self) -> TRACEDIV_W<0> {
        TRACEDIV_W::new(self)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn dbgcken(&mut self) -> DBGCKEN_W<8> {
        DBGCKEN_W::new(self)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    #[must_use]
    pub fn tracecken(&mut self) -> TRACECKEN_W<9> {
        TRACECKEN_W::new(self)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<12> {
        DBGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_dbgcfgr](index.html) module"]
pub struct RCC_DBGCFGR_SPEC;
impl crate::RegisterSpec for RCC_DBGCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_dbgcfgr::R](R) reader structure"]
impl crate::Readable for RCC_DBGCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_dbgcfgr::W](W) writer structure"]
impl crate::Writable for RCC_DBGCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_DBGCFGR to value 0x01"]
impl crate::Resettable for RCC_DBGCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
