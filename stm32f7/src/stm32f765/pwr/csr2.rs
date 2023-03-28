#[doc = "Register `CSR2` reader"]
pub struct R(crate::R<CSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR2` writer"]
pub struct W(crate::W<CSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR2_SPEC>;
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
impl From<crate::W<CSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPF1` reader - Wakeup Pin flag for PA0"]
pub type WUPF1_R = crate::BitReader<bool>;
#[doc = "Field `WUPF2` reader - Wakeup Pin flag for PA2"]
pub type WUPF2_R = crate::BitReader<bool>;
#[doc = "Field `WUPF3` reader - Wakeup Pin flag for PC1"]
pub type WUPF3_R = crate::BitReader<bool>;
#[doc = "Field `WUPF4` reader - Wakeup Pin flag for PC13"]
pub type WUPF4_R = crate::BitReader<bool>;
#[doc = "Field `WUPF5` reader - Wakeup Pin flag for PI8"]
pub type WUPF5_R = crate::BitReader<bool>;
#[doc = "Field `WUPF6` reader - Wakeup Pin flag for PI11"]
pub type WUPF6_R = crate::BitReader<bool>;
#[doc = "Field `EWUP1` reader - Enable Wakeup pin for PA0"]
pub type EWUP1_R = crate::BitReader<bool>;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin for PA0"]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin for PA2"]
pub type EWUP2_R = crate::BitReader<bool>;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin for PA2"]
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin for PC1"]
pub type EWUP3_R = crate::BitReader<bool>;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin for PC1"]
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin for PC13"]
pub type EWUP4_R = crate::BitReader<bool>;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin for PC13"]
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin for PI8"]
pub type EWUP5_R = crate::BitReader<bool>;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin for PI8"]
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
#[doc = "Field `EWUP6` reader - Enable Wakeup pin for PI11"]
pub type EWUP6_R = crate::BitReader<bool>;
#[doc = "Field `EWUP6` writer - Enable Wakeup pin for PI11"]
pub type EWUP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn wupf1(&self) -> WUPF1_R {
        WUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn wupf2(&self) -> WUPF2_R {
        WUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn wupf3(&self) -> WUPF3_R {
        WUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn wupf4(&self) -> WUPF4_R {
        WUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn wupf5(&self) -> WUPF5_R {
        WUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn wupf6(&self) -> WUPF6_R {
        WUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<8> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<9> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<10> {
        EWUP3_W::new(self)
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<11> {
        EWUP4_W::new(self)
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<12> {
        EWUP5_W::new(self)
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> EWUP6_W<13> {
        EWUP6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr2](index.html) module"]
pub struct CSR2_SPEC;
impl crate::RegisterSpec for CSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr2::R](R) reader structure"]
impl crate::Readable for CSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr2::W](W) writer structure"]
impl crate::Writable for CSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for CSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
