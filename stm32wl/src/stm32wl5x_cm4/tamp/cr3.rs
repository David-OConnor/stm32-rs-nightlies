#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub type ITAMP3NOER_R = crate::BitReader<ITAMP3NOER_A>;
#[doc = "ITAMP3NOER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3NOER_A {
    #[doc = "0: Internal tamper x event erases the backup registers"]
    Erase = 0,
    #[doc = "1: Internal tamper x event does not erase the backup registers"]
    NotErase = 1,
}
impl From<ITAMP3NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3NOER_A) -> Self {
        variant as u8 != 0
    }
}
impl ITAMP3NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3NOER_A {
        match self.bits {
            false => ITAMP3NOER_A::Erase,
            true => ITAMP3NOER_A::NotErase,
        }
    }
    #[doc = "Checks if the value of the field is `Erase`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ITAMP3NOER_A::Erase
    }
    #[doc = "Checks if the value of the field is `NotErase`"]
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        *self == ITAMP3NOER_A::NotErase
    }
}
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub type ITAMP3NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, ITAMP3NOER_A, O>;
impl<'a, const O: u8> ITAMP3NOER_W<'a, O> {
    #[doc = "Internal tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ITAMP3NOER_A::Erase)
    }
    #[doc = "Internal tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(ITAMP3NOER_A::NotErase)
    }
}
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub use ITAMP3NOER_R as ITAMP5NOER_R;
#[doc = "Field `ITAMP6NOER` reader - ITAMP6NOER"]
pub use ITAMP3NOER_R as ITAMP6NOER_R;
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub use ITAMP3NOER_R as ITAMP8NOER_R;
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub use ITAMP3NOER_W as ITAMP5NOER_W;
#[doc = "Field `ITAMP6NOER` writer - ITAMP6NOER"]
pub use ITAMP3NOER_W as ITAMP6NOER_W;
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub use ITAMP3NOER_W as ITAMP8NOER_W;
impl R {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<2> {
        ITAMP3NOER_W::new(self)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<4> {
        ITAMP5NOER_W::new(self)
    }
    #[doc = "Bit 5 - ITAMP6NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<5> {
        ITAMP6NOER_W::new(self)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<7> {
        ITAMP8NOER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
