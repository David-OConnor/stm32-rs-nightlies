#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSLEEP_D1` reader - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_D1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSLEEP_D1` writer - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTOP_D1` reader - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_D1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTOP_D1` writer - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBY_D1` reader - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_D1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBY_D1` writer - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TRACECLKEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TRACECLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `D1DBGCKEN` reader - D1 debug clock enable"]
pub type D1DBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `D1DBGCKEN` writer - D1 debug clock enable"]
pub type D1DBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `D3DBGCKEN` reader - D3 debug clock enable"]
pub type D3DBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `D3DBGCKEN` writer - D3 debug clock enable"]
pub type D3DBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TRGOEN_R = crate::BitReader<bool>;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TRGOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&self) -> DBGSLEEP_D1_R {
        DBGSLEEP_D1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&self) -> DBGSTOP_D1_R {
        DBGSTOP_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&self) -> DBGSTBY_D1_R {
        DBGSTBY_D1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsleep_d1(&mut self) -> DBGSLEEP_D1_W<0> {
        DBGSLEEP_D1_W::new(self)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_d1(&mut self) -> DBGSTOP_D1_W<1> {
        DBGSTOP_D1_W::new(self)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstby_d1(&mut self) -> DBGSTBY_D1_W<2> {
        DBGSTBY_D1_W::new(self)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<20> {
        TRACECLKEN_W::new(self)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W<21> {
        D1DBGCKEN_W::new(self)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W<22> {
        D3DBGCKEN_W::new(self)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgoen(&mut self) -> TRGOEN_W<28> {
        TRGOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
